from ipykernel.kernelapp import IPKernelApp
from .kernel import ErgKernel
IPKernelApp.launch_instance(kernel_class=ErgKernel)
