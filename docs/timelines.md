# Timelines

> This document is not finished. Please ignore for now.

In the 0.2.0 release, a Scene will have an array of Timeline objects that can be used to orchestrate the animation of Scene child views. Each child Displayable view also has a Layer that contains an optional Tween animation. This represents a change from the prior release where Tweek contained a collection of Timelines, which each contained one or more Tween objects.

