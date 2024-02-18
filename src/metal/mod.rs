extern crate metal;
extern crate objc;

use metal::*;
use objc::runtime::Object;
use std::mem::size_of;
use ark_std::rand;

const TEST_SIZE: usize = 100_000_000;

pub fn run() {
    // Create a Metal device
    let device = Device::system_default().expect("no device found");

    // Load the compiled Metal shader library
    let library_path = "src/metal/fp.metallib";
    let library = device.new_library_with_file(library_path).unwrap();

    // Obtain a reference to the kernels
    let add_function = library.get_function("addVectors", None).unwrap();
    let multiply_function = library.get_function("multiplyVectors", None).unwrap();

    // Create a command queue
    let command_queue = device.new_command_queue();

    // Generate data
    let num_elements = TEST_SIZE; // For example purposes
    let mut rng = rand::thread_rng();
    let input_a_data: Vec<u64> = (0..num_elements * 4).map(|_| rng.gen()).collect();
    let input_b_data: Vec<u64> = (0..num_elements * 4).map(|_| rng.gen()).collect();

    // Example: Define your input/output buffers
    let buffer_length = num_elements * size_of::<u64>() * 4; // Adjust based on your data structure
    let input_a = device.new_buffer_with_data(
        input_a_data.as_ptr() as *const _,
        buffer_length as u64,
        MTLResourceOptions::StorageModeShared,
    );

    let input_b = device.new_buffer_with_data(
        input_b_data.as_ptr() as *const _,
        buffer_length as u64,
        MTLResourceOptions::StorageModeShared,
    );
    let output = device.new_buffer(buffer_length as u64, MTLResourceOptions::StorageModeShared);

    // Create a compute command encoder
    let command_buffer = command_queue.new_command_buffer();
    let encoder = command_buffer.new_compute_command_encoder();
    let pipeline_state = device.new_compute_pipeline_state_with_function(&multiply_function).unwrap();
    encoder.set_compute_pipeline_state(&pipeline_state);
    encoder.set_buffer(0, Some(&input_a), 0);
    encoder.set_buffer(1, Some(&input_b), 0);
    encoder.set_buffer(2, Some(&output), 0);

    // Dispatch the compute kernel
    let threads_per_group = MTLSize { width: 64, height: 1, depth: 1 };
    let num_threadgroups = MTLSize { width: (num_elements as u64) / 64, height: 1, depth: 1 };
    
    // Start timing
    let start_time = std::time::Instant::now();
    
    encoder.dispatch_thread_groups(num_threadgroups, threads_per_group);
    encoder.end_encoding();

    // Commit the command buffer and wait for completion
    command_buffer.commit();
    command_buffer.wait_until_completed();
    
    // End timing and print the elapsed time
    let elapsed_time = start_time.elapsed();
    println!("Time elapsed for compute kernel dispatch and completion: {:?}", elapsed_time);

    // Read back and process the data from the 'output' buffer...
    // Example of reading back the output buffer data
    let output_ptr = output.contents() as *const u64;
    let output_slice = unsafe { std::slice::from_raw_parts(output_ptr, num_elements * 4) };

    // Print the first few elements of the output to verify
    println!("Output vector:");
    for i in 0..10 {
        println!("Element {}: {}", i, output_slice[i * 4]); // Printing only the first part of each u256 for simplicity
    }
}

use ark_bn254::Fr;
use ark_std::rand::Rng;
use ark_std::UniformRand;
use std::time::Instant;

pub fn ark_run() {
    let mut rng = ark_std::test_rng();

    // Generate TEST_SIZE * 2 ark_bn254::Fr elements in 2 different vectors
    let mut vec1 = Vec::with_capacity(TEST_SIZE);
    let mut vec2 = Vec::with_capacity(TEST_SIZE);
    for _ in 0..TEST_SIZE {
        vec1.push(Fr::rand(&mut rng));
        vec2.push(Fr::rand(&mut rng));
    }

    // Measure the time required to multiply them
    let start = Instant::now();
    for i in 0..TEST_SIZE {
        // let thing = vec1[i] * vec2[i];
        let thing = vec1[i] + vec2[i];
        core::hint::black_box(thing);
    }
    let duration = start.elapsed();

    println!("Time elapsed for multiplication: {:?}", duration);
}
