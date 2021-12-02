#!/usr/bin/env zsh

_dz=("子" "丑" "寅" "卯" "辰" "巳" "午" "未" "申" "酉" "戌" "亥")
now=$(date "+%H")
dz=$_dz[$[ ($now + 1) / 2 % 12 + 1]]"   時"
now=$now":"$[ $(date "+%M") + 1]
#convert $1 -font /home/hawk/.local/share/fonts/Noteworthy-Bold.ttf -fill white -pointsize 100 -draw "text 1800,980 '$now'" $2
convert $1 -font /home/hawk/.local/share/fonts/Noteworthy-Bold.ttf -gravity SouthEast -pointsize 200 -fill white -annotate +100+100 "$now" $2
convert $2 -font /usr/share/fonts/wps-office/FZLSK.TTF -gravity SouthEast -pointsize 200 -fill white -annotate +100+500 "$dz" $2
