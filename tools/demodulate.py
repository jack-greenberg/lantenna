from rtlsdr import RtlSdr
import numpy as np
import matplotlib.pyplot as plt
from scipy import fftpack, signal
from pylab import *

sdr = RtlSdr()

sdr.sample_rate = 1e6
sdr.center_freq = 250e6
sdr.gain = 'auto'

results = np.zeros(1000)

for i in range(len(results)):
    average = 0
    for j in range(10):
        samples = np.real(sdr.read_samples(1024))
        freqs, psd = signal.welch(samples)
        average = average + psd[0]

    results[i] = average/10
    average = 0

low = min(results)
hi = max(results)
mid = (low + hi) / 2

bits = np.zeros(len(results))

for i in range(len(results)):
    if results[i] > mid:
        bits[i] = 1
    else:
        bits[i] = 0

plt.plot(results)
plt.xlabel("Samples")
plt.ylabel("250 MHz Signal Power")

plt.show()
