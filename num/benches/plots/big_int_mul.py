import matplotlib.pyplot as plt

# Data
digits = [1,2,4,8,16,32,64,128,256,512,1024,2048,4096,8192]
naive = [0.05784,0.06767,0.07851,0.12438,0.21672,0.50168,1.4112,4.7608,17.282,65.938,260.49,1033.8,4104.9,17367]
karatsuba = [0.05864,0.06331,0.07809,0.12112,0.21809,0.50210,5.6846,23.726,76.319,236.82,795.04,2433.9,7176.6,22407]
fft = [0.36972,0.51541,0.77680,1.1949,2.0510,3.8085,7.5764,15.825,34.258,74.455,170.38,387.30,868.36,2134.6]

# Plot
plt.figure()
plt.plot(digits, naive, marker='o', label='Naive')
plt.plot(digits, karatsuba, marker='o', label='Karatsuba')
plt.plot(digits, fft, marker='o', label='FFT')

plt.xscale('log', base=2)
plt.yscale('log')

plt.xlabel('Digits (log2 scale)')
plt.ylabel('Time (microseconds, log scale)')
plt.legend()
plt.tight_layout()
plt.show()
