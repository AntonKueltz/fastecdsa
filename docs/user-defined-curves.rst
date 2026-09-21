User-Defined Curves
===================
As of version 1.5.1 construction of arbitrary curves in Weierstrass form
:math:`y^2 \equiv x^3 + ax + b \pmod{p}` is supported. I advise against using custom curves for any
security sensitive applications. In versions before release 4.0.0 no checks on the curve parameters
are done. In 4.0.0+ some sanity checks are applied -

- :code:`p` must be a non-even (not 2) prime
- The curve cannot be singular (discriminant equal to 0)
- :code:`(gx, gy)` must be a point on the curve
- :code:`q` must be prime

Exhaustive validation checks are not performed e.g., no check that :code:`q` is actually the order of
point :code:`(gx, gy)` is performed.

.. code:: python

    from fastecdsa.curve import Curve
    curve = Curve(
        name,  # (str): The name of the curve
        p,     # (int): The value of p in the curve equation.
        a,     # (int): The value of a in the curve equation.
        b,     # (int): The value of b in the curve equation.
        q,     # (int): The order of the base point of the curve.
        gx,    # (int): The x coordinate of the base point of the curve.
        gy,    # (int): The y coordinate of the base point of the curve.
    )
