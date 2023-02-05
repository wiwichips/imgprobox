#!/usr/bin/env node

const Jimp = require("jimp");

async function openImage(filename) {
    const img = await Jimp.read(filename); debugger;
    const buffer = 
    return img;
}

openImage('./pics/otter.png').then(console.log);
