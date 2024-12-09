msg = "hello pocketpy !"
print(msg)

# output msg hex code to stdout
def hex_print(s):
    for c in s:
        print(hex(ord(c)), end=' ')
    print("")

print("")
hex_print(msg)

