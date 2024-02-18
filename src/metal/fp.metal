#include <metal_stdlib>
using namespace metal;

struct u256 {
    uint64_t parts[4]; // Represents a 256-bit unsigned integer

    // Adds two u256 values, handling carry
    static u256 add(device const u256& a, device const u256& b) {
        u256 result;
        uint64_t carry = 0;
        
        for (int i = 0; i < 4; i++) {
            uint64_t partSumLow = a.parts[i] + b.parts[i] + carry;
            uint64_t partSumHigh = (a.parts[i] > partSumLow) ? 1 : 0;
            result.parts[i] = partSumLow; // Keep lower 64 bits
            carry = partSumHigh; // Carry over upper 64 bits for next addition
        }
        
        return result;
    }

    // Multiplies two u256 values
    static u256 multiply(device const u256& a, device const u256& b) {
        u256 result = {0};
        for (int i = 0; i < 4; i++) {
            uint64_t carry = 0;
            for (int j = 0; j < 4; j++) {
                if (i + j < 4) {
                    uint64_t partProductLow = a.parts[i] * b.parts[j] + result.parts[i + j] + carry;
                    uint64_t partProductHigh = (a.parts[i] > partProductLow) ? 1 : 0;
                    result.parts[i + j] = partProductLow; // Keep lower 64 bits
                    carry = partProductHigh; // Carry over upper 64 bits for next iteration
                }
            }
        }
        return result;
    }
};

kernel void addVectors(device u256* inputA [[ buffer(0) ]],
                       device u256* inputB [[ buffer(1) ]],
                       device u256* output [[ buffer(2) ]],
                       const uint id [[ thread_position_in_grid ]]) {
    output[id] = u256::add(inputA[id], inputB[id]);
}

kernel void multiplyVectors(device u256* inputA [[ buffer(0) ]],
                            device u256* inputB [[ buffer(1) ]],
                            device u256* output [[ buffer(2) ]],
                            const uint id [[ thread_position_in_grid ]]) {
    output[id] = u256::multiply(inputA[id], inputB[id]);
}