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

    # _avg = []

    for i in range(10):
        samples = np.real(sdr.read_samples(1024))

        freqs, psd = signal.welch(samples)

        _avg.append(psd[103])
        # results[i] = psd[103]

    results[i] = sum(_avg) / len(_avg)
    # _avg = []


# plt.plot(freqs, psd)
plt.plot(results)

# print(p)
# 
# print(p[0][np.where()])


# plt.plot((2/256) * np.abs(fftpack.fft(S)))


# plt.plot(results[0])
plt.show()
# show()
