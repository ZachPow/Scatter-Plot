use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
//use std::time::{SystemTime};
use std::f32;

#[derive(Resource)]
struct Values{
    slope: f32,
    intercept: f32,
    x_values: Vec<f32>,
    y_values: Vec<f32>,
}

struct Point{
    x: f32,
    y: f32,
}


fn main() {


    //this is the input data
    let x_values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
    let y_values = vec![7.0, 4.0, 3.0, 6.0, 3.0, 4.0, 2.0];

    //panic if the input data is not the same length
    assert!(x_values.len() == y_values.len());

    let slope = get_slope(&x_values, & y_values);

    let intercept = get_intercept(&x_values, &y_values, slope);

    println!("Slope: {} intercept {}", slope, intercept);

    App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(Values{x_values: x_values, y_values: y_values,
                     slope: slope, intercept: intercept  })
    .add_systems(Startup, (make_graph.before(make_spheres), make_spheres))
    .run();
}


fn get_intercept(x_values: &Vec<f32>, y_values: &Vec<f32>, slope: f32) -> f32{

    let x_sum = sum(&x_values);
    let y_sum = sum(&y_values);
    let length = x_values.len() as f32;

    //intercept is y sum - slope (x sum) all divided by n

    let intercept = (y_sum-slope*x_sum)/length;

    return intercept;
}

fn get_slope(x_values: &Vec<f32>, y_values: &Vec<f32>) -> f32{

    //slope = (mean difference of x)(mean difference of y) / (mean difference of x squared)

    let x_difference = mean_difference(&x_values);
    let y_difference = mean_difference(&y_values);
    
    let x_difference_squared = mean_difference_squared(&x_values);

    let mut sum = 0.0;
    for i in 0..x_difference.len(){
        sum += x_difference[i]*y_difference[i];
    }


    let numerator = sum;

    sum = 0.0;
    for i in 0..x_difference_squared.len(){
        sum += x_difference_squared[i];
    }

    let denominator = sum;

    return numerator/denominator;
}

fn sum(values: &Vec<f32>) -> f32{

    let mut sum: f32 = 0.0;

    for value in values.iter(){

        sum += value;
    }
    sum
}

//returns a vector that contains the difference between the 
//values and the mean: values[i]-mean
fn mean_difference(values: &Vec<f32>) -> Vec<f32>{

    let mut mean_difference_values: Vec<f32> = Vec::new();

    let mean = get_mean(&values);

    for value in values.iter(){
        mean_difference_values.push(value-mean);
    }

    mean_difference_values
}

//takes values and does each value minus the mean
//takes that value and squares it then adds it to the vec
fn mean_difference_squared(values: &Vec<f32>)-> Vec<f32>{
    let mut mean_difference_values: Vec<f32> = Vec::new();

    let mean = get_mean(&values);

    for value in values.iter(){
        mean_difference_values.push((value-mean).powf(2.0));
    }

    mean_difference_values
}
fn get_mean(values: &Vec<f32>) -> f32{

    let length = values.len() as f32;

    let mut sum: f32 = 0.0;

    for val in values.iter(){
        sum += val;
    }

    sum/length
}

fn make_graph(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>,
    values: Res<Values>){

    let mut line = Mesh2dHandle(meshes.add(Rectangle::new(600.0, 5.0)));

    let color = Color::hsl(296 as f32, 1.0, 0.5);

    commands.spawn(Camera2dBundle::default());

    //makes the bottom line of graph
    commands.spawn(MaterialMesh2dBundle {
        mesh: line.clone(),
        material: materials.add(color),
        transform: Transform::from_xyz(
            0.0,
            -300.0,
            0.0,
        ),
        
        ..default()
    });

    //makes the left side of the graph
    commands.spawn(MaterialMesh2dBundle {
        mesh: line.clone(),
        material: materials.add(color),
        transform: Transform::from_xyz(
            -300.0,
            0.0,
            0.0,
        ).with_rotation(Quat::from_rotation_z(f32::consts::PI/2.0)),
        ..default()
    }); 

    //get data neccessary to place regression line
    let result = get_line_data(&values);

    //x_start and y_start describe where the left end of the line is
    //tracking this is important because we need to shift the line
    let x_start = result.0;
    let y_start = result.1;
    let angle = result.2;
    let line_length = result.3;

    let y_scale = get_scale(&values.y_values);
    
    //shift values to move the line into position
    let x_shift = -300.0 - x_start;
    let y_shift = (-300.0+values.intercept*y_scale)-y_start;


    line = Mesh2dHandle(meshes.add(Rectangle::new(line_length, 5.0)));

    commands.spawn(MaterialMesh2dBundle {
        mesh: line.clone(),
        material: materials.add(color),
        transform: Transform::from_xyz(
            x_shift,
            y_shift,
            0.0,
        ).with_rotation(Quat::from_rotation_z(angle)),
        ..default()
    });
}

fn get_line_data(values: &Values) -> (f32, f32, f32, f32){

    let slope = values.slope;
    let intercept = values.intercept;

    let x_scale = get_scale(&values.x_values);
    let y_scale = get_scale(&values.y_values);

    let width = 600.0/x_scale;
    let height = 600.0/y_scale;

    let start = Point{x: 0.0, y: intercept};
    let end = Point{x: width, y: slope*width+intercept};

    println!("start {} {}", start.x, start.y);
    println!("end {} {}", end.x, end.y);

    println!("slope {}", (end.y-start.y)/(end.x-start.x));

    //y = mx + b
    //let y_distance = ((slope*width)+intercept)*y_scale;
    let y_distance = (end.y-start.y)*y_scale;
    let x_distance = (width)*x_scale;

    println!("y distance {}", y_distance);

    //use a^2 + b^2 = c^2 to solve for the length of the line
    let line_length = (x_distance.powf(2.0) + y_distance.powf(2.0)).sqrt();

    let angle;
    let start_y;
    let start_x;

    let half_of_line = line_length/2.0;

    angle = (y_distance/x_distance).atan();

    start_y = -angle.sin()*half_of_line;
    start_x = -angle.cos()*half_of_line;

    
    println!("width {} hieght {}", width, height);
    println!("length {} angle {}", line_length, angle);
    println!("start x {} start y {}", start_x, start_y);

    return (start_x, start_y, angle, line_length);
}
fn make_spheres(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>,
        values: Res<Values>){

    let x_origin = -300.0;
    let y_origin = -300.0;

    let x_scale = get_scale(&values.x_values);
    let y_scale = get_scale(&values.y_values);

    let sphere = Mesh2dHandle(meshes.add(Circle::new(10.0)));

    let color = Color::hsl(296 as f32, 1.0, 0.5);

    for i in 0..values.x_values.len(){
        println!("({}, {})", values.x_values[i], values.y_values[i]);

        commands.spawn(MaterialMesh2dBundle{
            mesh: sphere.clone(),
            material: materials.add(color),
            transform: Transform::from_xyz(
                x_origin + values.x_values[i] * x_scale,
                y_origin + values.y_values[i] * y_scale,
                0.0
            ),
    
            ..default()
        });
    }

    
    commands.spawn(MaterialMesh2dBundle{
        mesh: sphere.clone(),
        material: materials.add(color),
        transform: Transform::from_xyz(
            -300.0,
            209.0,
            0.0
        ),

        ..default()
    });
    
}
//calculates scale
//for example: 1 unit on the graph is 10 pixels
//the scale is determined by the biggest value in the vec
fn get_scale(values: &Vec<f32>) -> f32{

    let mut biggest = values[0];

    for value in values.iter(){
        
        if *value > biggest{
            biggest = *value;
        }
    }

    600.0/biggest
}

