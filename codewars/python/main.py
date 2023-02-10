def accum(s):
    # convert the string to lowercase
    s.lower()

    # the final string to concat to
    final = ""

    # loop over the string and depending on its index,
    # add the letter to the final string * index
    for i in range(len(s)):
        final += s[i].upper() + s[i].lower * i + "-"
    # remove the last dash
    final = final[:-1]

    # return the final string
    return final


# run case
print(accum("abcd"))
