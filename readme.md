用ffmpeg命令行合并B站下载的视频

用法  
'''Rust
cargo run in_path out_path
'''

ffmpeg命令  
'''Shell
ffmpeg -i video.m4s -i audio.m4s -c:v copy -c:a aac -strict experimental output.mp4
'''
