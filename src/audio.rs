#[cfg(not(target_arch = "wasm32"))]
use std::fs::File;
#[cfg(not(target_arch = "wasm32"))]
use std::io::BufReader;
#[cfg(not(target_arch = "wasm32"))]
use rodio::Sink;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::mpsc::{self, Sender};
#[cfg(not(target_arch = "wasm32"))]
use std::thread;

//https://github.com/RustAudio/rodio/issues/214
#[cfg(not(target_arch = "wasm32"))]
pub struct Audio {
    control_channel: Sender<(i32, String, i32)>,
}

pub const SINK_1: i32 = 0;
pub const SINK_2: i32 = 1;

pub const PLAY: i32 = 1;
pub const PAUSE: i32 = 2;
pub const LOAD: i32 = 0;

pub const NONE: &str = "";

#[cfg(not(target_arch = "wasm32"))]
impl Audio {

    pub fn new() -> Self {

        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {

            let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
            let sink1 = Sink::try_new(&stream_handle).unwrap();
            let sink2 = Sink::try_new(&stream_handle).unwrap();
            sink1.pause();
            sink2.pause();
            while let Ok(sinkid_name_stat) = rx.recv() {
                let (sinkid, name, stat) = sinkid_name_stat;
                if stat == LOAD {
                    let file = File::open(name).unwrap();
                    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
                    if sinkid == SINK_1 {
                        sink1.append(source);
                    }
                    else if sinkid == SINK_2 {
                        sink2.append(source);
                    }
                }
                else if stat == PLAY {

                    if sinkid == SINK_1 {
                        sink1.play();
                    }
                    else if sinkid == SINK_2 {
                        sink2.play();
                    }
                }
                else if stat == PAUSE {

                    if sinkid == SINK_1 {
                        sink1.pause();
                    }
                    else if sinkid == SINK_2 {
                        sink2.pause();
                    }
                }
            }
        });
        Self {
            control_channel: tx,
        }
    }

    pub fn play(&self, sinkid_name_stat: (i32, String, i32)) {
        self.control_channel.send(sinkid_name_stat).unwrap();
    }
}


//Run audio on web through js
#[cfg(target_arch = "wasm32")]
pub struct Audio;


#[cfg(target_arch = "wasm32")]
impl Audio {

    pub fn new() -> Self {

        //Pre load songs
        js! {
            var xepher = document.createElement("AUDIO");
            xepher.setAttribute("src", "xepher.mp3");
            xepher.setAttribute("id", "xepher");
            document.body.appendChild(xepher);

            var karin = document.createElement("AUDIO");
            karin.setAttribute("src", "karin.mp3");
            karin.setAttribute("id", "karin");
            document.body.appendChild(karin);  
        }
        Self {
        }
    }

    pub fn play(&self, sinkid_name_stat: (i32, String, i32)) {
        let (sinkid, name, stat) = sinkid_name_stat;
        if stat == PLAY {
            js! {
                var name = @{name};
                document.getElementById(name).play();
            }
        }
        else if stat == PAUSE {
            js! {
                var name = @{name};
                document.getElementById(name).pause();
            }
        }
    }
}