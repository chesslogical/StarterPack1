

Showing cpu core temp in Linux. 


Please note that this is a very basic example, and there are many enhancements and error checks you could add to make it more robust and user-friendly. Additionally, the paths and methods to retrieve CPU temperature can vary based on the system and hardware, so this might not work on all Linux systems without modification.



it only shows 3 cores???

The code example I provided reads temperature information from the /sys/class/thermal/thermal_zone*/temp directory, which is a common location for thermal sensors in Linux. However, this directory doesn’t always map one-to-one with CPU cores, as the actual arrangement depends on the specific hardware and how the sensors are exposed by the Linux kernel.

There are a few reasons why you might be seeing temperatures for only three cores:

1. Sensor Location and Availability:
The thermal zones might not correspond directly to CPU cores. Some CPUs might have one sensor for the whole package, some might have one per core, and others might have multiple sensors per core.

2. Kernel and Driver Support:
The Linux kernel needs to have the proper drivers to communicate with the CPU’s thermal sensors. If the driver isn’t loaded, or if there’s no driver support for your specific CPU’s thermal sensors, then the temperature information might not be available.

3. Hardware Differences:
Different CPUs and motherboards have different hardware monitoring capabilities. Some might expose temperature information for each core, while others might not.

Alternative Approach: hwmon
Another common place to find CPU temperature information is in the /sys/class/hwmon/hwmon*/temp*_input files. These are provided by the hwmon (hardware monitoring) subsystem in the Linux kernel and can sometimes provide more detailed information.

You might modify the program to also search in this location, or use tools like lm-sensors in Linux to help identify where the temperature information for your CPU cores is located.

If you know the specific location of the temperature information for your CPU cores, you can modify the program to read from those files directly.

Using sensors Command
The sensors command (from the lm-sensors package) is a commonly used tool in Linux for displaying temperatures and other hardware health information. You might want to use this tool to quickly check if the temperature information for all CPU cores is available on your system:


sudo apt-get install lm-sensors
sudo sensors-detect
sensors
The sensors command should display all available temperature information, and you can use its output to determine the correct paths to use in your Rust program.





