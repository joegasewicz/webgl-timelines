# webgl-timelines
Display timelines in the DOM

### Usages
Add a div to your html body with an id of `webgl-timeline`.
```
<div id="webgl-timeline"></div>
```

To create a new time:
```
newTimeline();
```

## Contributing

#### Build Web package:
```
wasm-pack build --target web
```


#### Build Npm package:
```
wasm-pack build --target bundler --out-dir lib
```
