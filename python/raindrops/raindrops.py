def convert(number):
    result = "".join(
        v for k, v
        in ((3, "Pling"), (5, "Plang"), (7, "Plong"))
        if not number % k
    )
    return result or str(number)
