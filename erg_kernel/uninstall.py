from jupyter_client.kernelspec import KernelSpecManager

def uninstall_my_kernel_spec():
    print('Uninstalling IPython kernel spec')
    KernelSpecManager().remove_kernel_spec('erg')

if __name__ == '__main__':
    uninstall_my_kernel_spec()
