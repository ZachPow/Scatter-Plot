# Scatter Plot Visualization

## Usage
Given two vectors (x and y values) this program will create a graph and plot the points.
The program will then calculate the regression line and add that to the graph.

## Bevy
This program uses bevy for all of its graphics. Bevy is a game engine commonly used with the rust programming language. 
Bevy @version "0.13.2"

## Steps 
- Set up the graph (left and bottom lines)
- Do calculations on the data to determine slope, y-intercept, angle, and other things important to graphing the regression line
- Using this information to add the line to the graph
- Graph the points

## Specifics of the code
### Scaling
- This program scales the ratio of pixel to units on the graph
- Orignally the scaling is a 1:1 ratio meaning the point (2, 5) is only a couple pixels away from the origin of the graph
- To fix this the program determines an appropriate scaling. Addionally the program scales the y and x axis seperately
- The program takes the total width of the graph (600 pixels) and divides it by the largest x value. Example: if 10 was the largest x value the x scaling would be set to 60 and that point would be shifted 600 pixels to the right but would represent only 10 units on the graph. This would make the ratio of pixels to graph units 60:1 
- The program uses the same technique to determine the scaling for the y axis

### Positioning in Bevy
- Bevy places all objects at (0, 0)
- (0, 0) is the middle of the screen
- Bevy also rotates all objects by the center
- Most of the complexity of this project came from how Bevy centers objects in the middle and the scaling. 
