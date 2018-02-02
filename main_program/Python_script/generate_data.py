import sys
import string
import matplotlib.pyplot as plt
from os import listdir
from os.path import isfile, join

basedir_file = "../docs_lab_1_techno_multimedia/"
text_filename = "long_text1.txt"
text2_filename = "long_text2.txt"
text3_unicode_filename = "unicode_random.txt"
pic1_filename = "image1.jpg"
pic2_filename = "image2.jpg"
pic3_filename = "image3.jpg"



def update_dico(key):
    if key in dictionary:
        dictionary[key] += 1
    else:
        dictionary[key] = 1


for filename in listdir(basedir_file + "calgary_data_set"):
    dictionary = dict()


    with open(basedir_file + "calgary_data_set/" + filename, 'rb') as f:

        while 1:
            byte_s = f.read(1)
            if not byte_s:
                break
            i = ord(byte_s)
            binary = "{0:b}".format(i)
            update_dico(binary)

    f.close()
    print(filename)
    plt.bar(range(len(dictionary)), list(dictionary.values()), align='center')
    plt.xticks(range(len(dictionary)), list(dictionary.keys()), rotation=90)
    plt.show()
