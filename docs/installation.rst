Installation
============
The only actively supported operating systems at this time are most Linux distros and OS X.

You can use pip: :code:`$ pip install fastecdsa` or clone the repo and use
:code:`$ python setup.py install`. Note that you need to have a C compiler (you can check this via
e.g. :code:`$ which gcc` or :code:`$ which clang`). You  also need to have  GMP_ on your system
as the underlying C code in this package includes the :code:`gmp.h` header  (and links against gmp
via the :code:`-lgmp` flag).

Installing Dependencies
-----------------------

Ubuntu / Debian
~~~~~~~~~~~~~~~

.. code:: bash

    $ sudo apt-get install gcc python-dev libgmp3-dev

RHEL / CentOS
~~~~~~~~~~~~~

.. code:: bash

    $ sudo yum install gcc python-devel gmp-devel

.. _GMP: https://gmplib.org/
