用ffmpeg命令行合并B占下载的视频

ffmpeg命令
ffmpeg -i video.m4s -i audio.m4s -c:v copy -c:a aac -strict experimental output.mp4

