# Data Scaling

Because .wav files are of such (and should be) a high sample rate, it is quite hard to align it to lower sample rate facial tracking devices. As such, this system takes in velocity, acceleration, and jerk of landmark points to artificially increase (or potentially decrease) sampling rates of articulatory methods. It monitors uncertainty to allow you, the user, to change how much the data is scaled to ensure the quality/uncertainty of data stays reasonable.

## How data is scaled

### Velocity
By understanding the velocity of landmarks between 2 frames, we can help fill in where the landmark may be in between frames because we know the distance it would have theoretically travelled in between the two frames. 

### Acceleration
By understanding the acceleration of landmarks between 2 frames, we now don't just infer where it should be linearly through velocity, but how quick it is changing in acceleration. This helps us predict where the point should be more accurately than a linear velocity. (I.e. if the landmark is decelarting, it is likely it would fall short of where we would predict it to be based purely off velocity). 

### Jerk
Though less prevelant in fluent speech, Jerk allows us to have one more dimension for scaling data by telling us how the accelration of the mouth is changing. However, I am going to predict this will increase the uncertainty a fair bit, which is relative to your tracking method. (i.e. +/- 2px on 4k video is not the same as 1080p)



The math hasn't been implemeneted, this file is more expressing an idea.