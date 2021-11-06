# Homecloud Photos

Self hosted photo library. 

## Goals

I started this project after not finding any self hosted photo libraries that
supported everything I wanted. I've been spoiled by services like Google Photos
and its extremely intuitive search. This project is an attempt to build a high
performance, feature rich photo library that is as simple to use and maintain as
Google Photos.

### High Performance

Google Photos has a very smooth experience regardless of library size. When you
scroll, the UI remains responsive and thumbnails flow by smoothly even when 
scrolling quickly. This is something I hope to replicate. Homecloud Photos uses
the following strategies to acomplish this:

- Two stages of thumbnails for low quality previews during fast scrolling, and
  higher quality ones when scrolling slower.
- Gallery view built on a virtual list for high performance scrolling of large
  libraries.

I could mention here that performance is why this project is written in rust,
but that would be a lie. It is true that I hope rust helps deliver a responsive
user experience, even on resource constrained hardware like a Raspberry Pi. But
the real reason I choose rust is because I wanted to learn it, and wanted a
project to facilitate that.

### Feature Rich

- All image processing done on device.
- Object and facial recognition.
- Parsing and indexing of exif data.
- Dominant colors detection.
- Searchable by everything listed above.

### Simplicity

One of the main goals of this project is to provide a user experience that is
similar to that of Google or Apple photos provide. There are certain features
that can be tweaked and changed, but the defaults should be setup in a sensible
way.

## Future

If this project gains traction, my intent is to expand the Homecloud ecosystem
to other services that do not have easy to use self hosted options. This
initially started when I decided to get more serious with my homelab setup.
I found that while some services like file syncing and remote access have great
solutions like syncthing and tailscale respectively, other services were more
complicated to setup and run. Homelabs are fun environments to learn and tinker,
but for certain services I'm more interested in using the tool than I am
tinkering with it. This is a driving goal of this project. Anything in the
Homecloud ecosystem will be designed to be as simple as possible.
