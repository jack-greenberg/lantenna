fs = 1000; % this is the sample rate
fc = 250.01e6; % this is the center frequency

bit_rate = 5; % 5 Hz

x = zeros(10e6,1); % empty vector to store data

% create object for RTL-SDR receiver
rx = comm.SDRRTLReceiver('CenterFrequency',fc, 'EnableTunerAGC', false, 'TunerGain', 35,  'SampleRate', fs);

counter = 1; % initialize a counter
while(counter < length(x)) % while the buffer for data is not full
    rxdata = rx();   % read from the RTL-SDR
    x(counter:counter + length(rxdata)-1) = rxdata; % save the samples returned
    counter = counter + length(rxdata); % increment counter
end

% the data are returned as complex numbers
% separate real and imaginary part, and remove any DC offset
y_I = real(x);% -mean(real(x));


% sample rate is 1000 Hz, each window will be

for i = 0:length(x)/1000-1
    window = y_I((i*1000)
end
