#!/usr/bin/env zsh

# https://wallhere.com/zh/tag/894?page=2&format=json
# https://wallhere.com/zh/wallpapers?q=狗&page=2&format=json
tag=36
#base_page="https://wallhere.com/zh/tag/$tag?page=$2&pageSize=1&format=json"
base_page="https://wallhere.com/zh/wallpapers?q=狗&page=$2&pageSize=1&format=json"
# https://wallhere.com/zh/wallpaper/1500657
data_id=$(curl $base_page | jq .data | sed  -e 's/\\r\\n//g' -e 's/\\//g' | grep -Po '(?<=data-id=")(\d+?)(?=")' | head -n1)

img_url="https://wallhere.com/zh/wallpaper/$data_id"
img_src=$(curl $img_url | grep current-page-photo | grep -Po '(?<=href\=")(.*?)(?=")')

wget $img_src -O $1
echo $1
