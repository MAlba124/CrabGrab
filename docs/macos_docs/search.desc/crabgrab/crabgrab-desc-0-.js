searchState.loadedDescShard("crabgrab", 0, "A cross-platform screen/window/audio capture library\nEnumeration of capturable items\nThe actual capture stream and related constructs\nExtension features\nAudio and video frames\nPlatform-specific extensions\nEverything\nGeometry types\nAll capturable windows, but no displays\nRepresents an application with capturable windows\nA collection of capturable content (windows, screens)\nRepresents an error that occurred when enumerating …\nSelects the kind of capturable content to enumerate\nRepresents a capturable display\nAn iterator over capturable displays\nRepresents a capturable application window\nSelects the kind of windows to enumerate for capture\nAn iterator over capturable windows\nAll capturable displays, but no windows\nEverything that can be captured\nOnly normal windows and displays\nOnly normal windows - no modal panels, not the dock on …\nGets the application that owns this window\nDesktop windows are elements of the desktop environment, …\nGet an iterator over the capturable displays\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGets the “identifier” of the application\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nWhether this filter allows any capturable content\nChecks whether an application is visible (on-screen, not …\nGets the friendly name of the application\nRequests capturable content from the OS\nCreate a new content filter with the given filtering …\nWhether to restrict to onscreen windows\nGets the process id of the application\nGets the virtual screen rectangle of the window\nGets the virtual screen rectangle of this display\nGets the title of the window\nGet an iterator over the capturable windows\nThe stream was already stopped\nOne plane, 4 channels, 10 bits per color channel, two bits …\nThis event is produced when the stream receives a new …\nConfiguration settings for audio streams\nOne plane, 4 channels, 8 bits per channel: { b: u8, g: u8, …\nRepresents programmatic capture access\nConfiguration settings for a capture stream\nRepresents an error creating the capture config\nThe pixel format of returned video frames\nRepresents an active capture stream\nThis event is produced once at the end of the stream\nTwo planes:\nThis event is produced when the stream goes idle - IE when …\nThe buffer count is out of the valid range for the …\nThis represents an error when creating a capture stream\nThis represents an error during a stream, for example a …\nRepresents an event in a capture stream\nThis represents an error while stopping a stream\nRequested features are not authorized\nThe supplied pixel format is unsupported by the …\nThe pixel format is unsupported by the implementation\nTwo planes:\nThis event is produced when the stream receives a new …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nStart a new capture stream with the given stream callback\nCreates a new audio capture config with default settings:\nPrompt the user for permission to capture content\nStop the capture\nGets the implementation’s supported pixel formats\nTest whether the calling application has permission to …\nConfigure the buffer count - the number of frames in the …\nCreate a capture configuration for a given capturable …\nConfigure the output texture size - by default, this will …\nConfigure whether the cursor is visible in the capture\nSupply a Wgpu device to the config, allowing the …\nCreate a capture configuration for a given capturable …\nFrame to Bitmap conversion (requires <code>bitmap</code> feature)\nFrame -&gt; IOSurface conversion (requires <code>iosurface</code> feature)\nFrame -&gt; Metal Texture conversion (requires <code>metal</code> feature)\nScreenshot utility function (requires <code>screenshot</code> feature)\nFrame -&gt; Wgpu Texture conversion (requires <code>wgpu</code> feature)\nBitmap data in the Argb2101010 format\nBitmap data in the Bgra8888 format\nBitmap data in the CbCr Chroma/u8x2 format\nBitmap data in the Luma/u8 format\nBitmap data in the RgbaF16x4 format\nA Bitmap with boxed-slice image data\nA bitmap image of the selected format\nA Rgba1010102 format bitmap\nA Bgra8888 format bitmap\nA pool of frame bitmaps\nA RgbaF16x4 format bitmap\nA YCbCr image, corresponding to either V420 or F420 pixel …\nLuma: [0, 255], Chroma: [0, 255]\nA pooled bitmap, belinging to it’s creator BitmapPool. …\nA bitmap with booled images as bitmap data\nLuma: [16, 240], Chroma: [0, 255]\nA video frame which can produce a bitmap\nRepresents an error while generating a frame bitmap\nThe video range for a YCbCr format bitmap\nFree all pooled bitmaps - this happens automatically on …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCreate a bitmap image from this frame. This usually …\nGet a pooled bitmap, waiting for one to become available …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreate a new frame bitmap pool, limited to <code>max</code> pooled …\nCreate a new bitmap pool with an initial <code>capacity</code> and …\nTry and get a pooled bitmap using the given bitmap pool, …\nRepresents an error when getting the IOSurface behind this …\nA MacOS IOSurface instance\nA video frame which can inter-operate with any MacOS GPU …\nThere was no image buffer in this frame\nThere was no IOSurface in the frame’s image buffer\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet the IOSurface representing the video frame’s texture\nGets the raw IOSurfaceRef\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe Chrominance (CbCr, Blue/Red) plane for a YCbCr format …\nThe Luminance (Y, Brightness) plane for a YCbCr format …\nRepresents an error getting the texture from a video frame\nA capture stream which inter-operates with Metal\nA video frame which can be used to create metal textures\nIdentifies planes of a video frame\nThe single RGBA plane for an RGBA format frame\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet the metal device used for frame capture\nGet the texture for the given plane of the video frame\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nRepresents an error while taking a screenshot\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nTake a screenshot of the capturable content given a …\nThe Chrominance (CbCr, Blue/Red) plane for a YCbCr format …\nThe requested plane isn’t valid for this frame\nThe Luminance (Y, brightness) plane for a YCbCr format …\nthe backend texture couldn’t be fetched\nNo Wgpu device was supplied to the capture stream\nThe single RGBA plane for an RGBA format frame\nA capture config which can be supplied with a Wgpu device\nA capture stream which may have had a Wgpu device instance …\nRepresents an error getting the texture from a video frame\nA video frame which can be used to create Wgpu textures\nIdentifies planes of a video frame\nReturns the argument unchanged.\nReturns the argument unchanged.\nGets the Wgpu device referenced by device wrapper supplied …\nGets the Wgpu device wrapper supplied to …\nGet the texture for the given plane of the video frame\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nRepresents an error getting the data for an audio channel\nThe number of audio channels to capture\nRepresents audio channel data in an audio frame\nWraps a “slice” of audio data for one channel, …\nA frame of captured audio\nThe rate to capture audio samples\nA frame of captured video\nGet the data buffer for the captured audio channel\nGet the Instant that this frame was delivered to the …\nGet the channel count of the captured audio\nGet the rectangle of the frame representing containing the …\nGet the dpi of the contents of the frame (accounting for …\nGet the duration of this audio frames\nGet the sequence id of this frame (monotonically …\nGet the sequence id of this video frame (monotonically …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet the nth sample for this channel data\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nGet the length of this sample buffer\nGet the time since the start of the stream that this audio …\nGet the time since the start of the stream that this frame …\nGet the sample rate of the captured audio\nGet the raw size of the frame\nMacos-specific extensions\nAutomatically select the resolution type\nSelect the highest available capture resolution (usually …\nMac OS specific extensions for audio capture configs\nMac OS specific extensions for capture content filters A …\nMac OS specific extensions for capturable windows A …\nMac OS specific extensions for capture configs\nMac OS “resolution type” The “resolution type” of …\nMac OS “window level” Represents the “window level”…\nOne linear screen unit per pixel, IE the “virtual …\nReturns the argument unchanged.\nReturns the argument unchanged.\nTry and convert the given CGWindowID to a capturable …\nGet the native window id for this capturable window. This …\nGet the window layer of this window\nGet the window level of this window\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nExclude windows who’s applications have the provided …\nExclude windows with the given CGWindowIDs\nSet the maximum capture frame-rate\nSet the metal device to use for texture creation\nSet the resolution type of the capture. Does nothing on …\nSet whether or not to scale content to the output size\nSet the range of “window levels” to filter to …\nRepresents a 2D point\nRepresents an axis-aligned rectangle\nRepresents a 2D size\nThe point at (0, 0)\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nscale the size uniformly by some value\nScale the point uniformly by some value\nScale the rectangle uniformly\nscale the size non-uniformly in x and y\nScale the point non-uniformly in x and y\nScale the rectangle non-uniformly in x and y")