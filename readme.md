# mvb - Minimum viable blogging engine

This application provides a simple way to host a markdown powered blog. All pages and resources are stored in a simple tree structure matching the following pattern:

    Indiviual pages: pages/[parent]/[id]-[slug]/content.md
    Page resources: pages/[parent]/[id]-[slug]/resource

Example pages are provided demonstating the available functionality.

## FAQ

### Q. This is written in rust, so it must be very fast, right?

Nope. This is actually quite slow. While rust as a language certainly has the potential to be fast, I've made no attempt to make this spectacularly quick. It serves static content, so I'm counting on a web server in front of this caching the hot pages.