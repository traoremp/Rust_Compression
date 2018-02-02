import sys
import string
import matplotlib.pyplot as plt

basedir_file = "../docs_lab_1_techno_multimedia/"
text_filename = "long_text1.txt"
text2_filename = "long_text2.txt"
text3_unicode_filename = "unicode_random.txt"
pic1_filename = "5373584-abstract-picture.jpg"
pic2_filename = "a80f04d3ac7b69dbf5b3b1fc8dd24489.jpg"
pic3_filename = "electricprisms1913_0.jpg"

dictionary = dict()


def update_dico(key):
    if key in dictionary:
        dictionary[key] += 1
    else:
        dictionary[key] = 1


with open(basedir_file + pic3_filename, 'rb') as f:

    while 1:
        byte_s = f.read(1)
        if not byte_s:
            break
        i = ord(byte_s)
        binary = "{0:b}".format(i)
        update_dico(binary)

f.close()

plt.bar(range(len(dictionary)), list(dictionary.values()), align='center')
plt.xticks(range(len(dictionary)), list(dictionary.keys()), rotation=90)
plt.show()
