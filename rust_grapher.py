import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import sys

#Copy from the Rust file
dt = 0.005
END_TIME = 10.0

#Defines a dictionary to color the subplots
color_dict = {
    0: "r",
    1: "g",
    2: "b"
}

#Defines a dictionary for the different kinematic information
dim_dict = {
    0: "Position [m]",
    1: "Velocity [m/s]",
    2: "Acceleration [m/s^2]"
}

#Takes a list of lists lst and returns a list of the ith element of each sublist
def Extract(lst, i):
    return [sub[i] for sub in lst]

#Reads the data from the file and puts it into an array
try:
    dataframe = pd.read_excel('kinematics_output.xlsx')
except:
    print("Please close the output file and try again.")
data_array = np.array(dataframe)

#Seperates out the different kinematic data, combines them into a single 2d array, and gets time
positions = Extract(data_array, 0)
velocities = Extract(data_array, 1)
accels =  Extract(data_array, 2)
complete_data = [positions, velocities, accels]
time = np.arange(0., END_TIME, dt)

#Creates a figure and axes with a certain layout
fig, axd = plt.subplot_mosaic([['0', '1'],
                                ['2', '2']],
                                layout="constrained")

#Creates and plots each set of data
for k, ax in axd.items():
    ax.plot(time, complete_data[int(k)], color_dict[int(k)])
    ax.set_xlabel("Time [s]")
    ax.set_ylabel(dim_dict[int(k)])

plt.show()