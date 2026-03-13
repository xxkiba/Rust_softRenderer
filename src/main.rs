//#![windows_subsystem = "windows"]

// std::mem for  mem::size_of, computeing the size of the FrameBuffer structure,
//time::Instant for measuring time,
// sync::OnceLock for lazy initialization when filling the FrameBuffer structure.
use std::{mem,time::Instant,sync::OnceLock};

// std::sync::Mutex for synchronizing access to FrameBuffer structure.
use std::sync::Mutex;
use log::{info, warn, error}; // log crate for logging information, warnings, and errors.

use windows::{
    core::*,                                                //Result, w!, PCWSTR, etc.
    Win32::{Foundation::*,                                  //HWND handle, LRESULT, SetDIBitToDevice,FrameBuffer, etc.
        Graphics::Gdi::*,                                   //GDIPixelFormat, GetDIBits, BITMAPINFO, BITMAPINFOHEADER, RGBQUAD, etc.
        System::LibraryLoader::GetModuleHandleW,            //Get current program's module handle. equivalent to C++Winmain's hInstance.
        UI::WindowsAndMessaging::*,                         //Windows management, CreateWindowExW, RegisterClassExW, etc.
    },
};

mod scene; //Scene module for rendering the scene, currently empty but can be expanded later.
mod float4; //Float4 module for representing 4D vectors, currently only contains the Float4 struct but can be expanded later.
mod boundingbox; //BoundingBox module for representing 2D and 3D bounding boxes, currently contains BoundingBox2D and BoundingBox3D structs but can be expanded later.
mod matrix3; 
mod matrix4; 
mod texture;
mod staticmesh; //StaticMesh module for loading and representing static meshes, currently contains StaticMesh struct and Vertex struct but can be expanded later.
//framebuffer
#[derive(Debug)]
pub struct FrameBuffer{
    hdc: HDC, // Handle to device context, used for drawing on the window.
    width: i32,
    height: i32,
    color_buffer: Vec<u32>, //ARGB format, 4 channels
    depth_buffer: Vec<f32>, //depth buffer, 1 channel
}


unsafe impl Send for FrameBuffer {} //Allow FrameBuffer to be sent across threads, as it contains raw pointers (HDC).
unsafe impl Sync for FrameBuffer {} //Allow FrameBuffer to be shared across threads, as it contains raw pointers (HDC).

impl FrameBuffer {
    //Create a new FrameBuffer with the specified width and height.
    pub fn new(hdc: HDC, width: i32, height: i32) -> Self {
        let size = (width * height) as usize;
        Self {
            hdc,
            width,
            height,
            color_buffer: vec![0u32; size], //Initialize color buffer with black pixels.
            depth_buffer: vec![1.0f32; size], //Initialize depth buffer with infinity.
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: u32) {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return; 
        }
        let index = (y * self.width + x) as usize;
            self.color_buffer[index] = color; //Update color buffer.
    }

    pub fn set_depth(&mut self, x: i32, y: i32, depth: f32) {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return; 
        }
        let index = (y * self.width + x) as usize;
        self.depth_buffer[index] = depth; //Update depth buffer.
    }

    pub fn get_depth(&self, x: i32, y: i32) -> f32 {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return 1.0f32; //Out of bounds, return infinity.
        }
        let index = (y * self.width + x) as usize;
        self.depth_buffer[index] //Return depth value.
    }

    pub fn clear(&mut self, color: u32) {
        self.color_buffer.fill(color); //Fill color buffer with the specified color.
        self.depth_buffer.fill(1.0f32); //Reset depth buffer to infinity.
    }

    pub fn present(&self) {
        unsafe {
            let bmi = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: mem::size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: self.width,
                    biHeight: self.height,
                    biPlanes: 1,
                    biBitCount: 32,
                    biCompression: BI_RGB.0,
                    biSizeImage: 0,
                    biXPelsPerMeter: 0,
                    biYPelsPerMeter: 0,
                    biClrUsed: 0,
                    biClrImportant: 0,
                },
                bmiColors: [RGBQUAD {
                    rgbBlue: 0,
                    rgbGreen: 0,
                    rgbRed: 0,
                    rgbReserved: 0,
                }],
            };

            SetDIBitsToDevice(
                self.hdc,
                0,
                0,
                self.width as u32,
                self.height as u32,
                0,
                0,
                0,
                self.height as u32,
                self.color_buffer.as_ptr() as *const _, //equivalanet to void* in C++, pass the pointer to the color buffer.
                &bmi,
                DIB_RGB_COLORS,
            );
        }
    }
    
}


const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;
static FRAME_BUFFER: OnceLock<Mutex<FrameBuffer>> = OnceLock::new(); //Global FrameBuffer instance

pub fn set_pixel(x: i32, y: i32, r: u8, g: u8, b: u8, a: u8) {
    let color = ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
    if let Some(frame_buffer) = FRAME_BUFFER.get(){//FRAME_BUFFER.get returns an Option, Some(Mutex<FrameBuffer>) or None if not initialized.

        // .lock() blocks until the lock is acquired, then returns:
        //   Ok(MutexGuard<FrameBuffer>)  → lock acquired successfully
        //   Err(PoisonError)             → another thread panicked while holding this lock
        // .unwrap() extracts the MutexGuard from Ok, or panics if the mutex is poisoned
        // MutexGuard automatically releases the lock when it goes out of scope
        frame_buffer.lock().unwrap().set_pixel(x, y, color); //Lock the mutex and set the pixel.
    }
}

pub fn clear(r: u8, g: u8, b: u8, a: u8) {
    let color = ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
    if let Some(frame_buffer) = FRAME_BUFFER.get(){
        frame_buffer.lock().unwrap().clear(color); //Lock the mutex and clear the framebuffer with the specified color.
    }
}

pub fn set_depth(x: i32, y: i32, depth: f32) {
    if let Some(frame_buffer) = FRAME_BUFFER.get(){
        frame_buffer.lock().unwrap().set_depth(x, y, depth); //Lock the mutex and set the depth value.
    }
}

pub fn get_depth(x: i32, y: i32) -> f32 {
    if let Some(frame_buffer) = FRAME_BUFFER.get(){
        return frame_buffer.lock().unwrap().get_depth(x, y); //Lock the mutex and get the depth value.
    }
    1.0f32 //If FrameBuffer is not initialized, return infinity.
}

extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT
{
    unsafe {
        match msg {
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            WM_ERASEBKGND => {
                LRESULT(1)
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}

fn main() -> Result<()> {
    
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .init();
    info!("Starting Soft Renderer...");

    unsafe {
        // Create the window and initialize the framebuffer.
        let hinstance = GetModuleHandleW(None)?; //Get current program's module handle. equivalent to C++Winmain's hInstance.
        info!("Module handle obtained: {:?}", hinstance);

        // Register the window class.
        let window_class = w!("SoftRendererWindow");
        let wc = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            hInstance: hinstance.into(),
            lpszClassName: window_class,
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hbrBackground: HBRUSH(GetStockObject(BLACK_BRUSH).0),
            cbClsExtra: 0,
            cbWndExtra: 0,
            lpszMenuName: PCWSTR::null(),
            hIcon: LoadIconW(None, IDI_APPLICATION)?,
        };

        RegisterClassW(&wc);
        log::debug!("Window class registered");

        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            window_class,
            w!("Soft Renderer"),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            WIDTH,
            HEIGHT,
            None,
            None,
            hinstance,
            None,
        )?;
        info!("Window created: {:?}", hwnd);

        let _ = ShowWindow(hwnd, SW_SHOW);
        info!("Window shown, entering main loop");
        let hdc = GetDC(hwnd);
        if hdc.is_invalid() {
            panic!("Failed to get device context");
        }

        FRAME_BUFFER.set(Mutex::new(FrameBuffer::new(hdc, WIDTH, HEIGHT)))
            .expect("Failed to initialize frame buffer");


        scene::init(WIDTH, HEIGHT);
        let mut frame_count = 0u64;
        let mut last_frame_time = Instant::now();
        let mut msg = MSG::default();
        'main_loop: loop{
            while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).into() {
                if msg.message == WM_QUIT {
                    info!("Received WM_QUIT, exiting main loop");
                    break 'main_loop;
                }
                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            let frame_start = Instant::now();            



            scene::render(last_frame_time.elapsed().as_secs_f64()); 

            FRAME_BUFFER.get().unwrap().lock().unwrap().present(); //Present the framebuffer to the window.
            
            
            last_frame_time = frame_start;
            frame_count += 1;
            // if frame_count % 60 == 0 {
            //         info!("Frames: {}, delta: {:.4}s", 60, delta_time.as_secs_f32());
            // }
            
        }

        info!("Total frames rendered: {}", frame_count);
        ReleaseDC(hwnd, hdc); //Release the device context when done.
    }



    Ok(())
}
