# Make sure you have rclone configured with a section for cloudflare
rclone copy -P hitsigst-songs/ cloudflare:hitrelease/
# To list all items in the bucket:
# rclone tree cloudflare:hitrelease
