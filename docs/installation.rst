installation
============
You can use pip: :code:`$ pip install fastecdsa` or clone the repo and use
:code:`$ python setup.py install`. Note that you need to have a C compiler.
You  also need to have GMP_ on your system as the underlying
C code in this package includes the :code:`gmp.h` header (and links against gmp
via the :code:`-lgmp` flag). GMP can be installed as follows:

Ubuntu / Debian
~~~~~~~~~~~~~~~

.. code:: bash

    $ sudo apt-get install python-dev libgmp3-dev

RHEL / CentOS
~~~~~~~~~~~~~

.. code:: bash

    $ sudo yum install python-devel gmp-devel

.. _GMP: https://gmplib.org/
