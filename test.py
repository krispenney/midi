import sys
sys.path.append('/usr/local/lib/python3.7/site-packages')

import mido
import time

outport = mido.open_output('VirtualDevice Bus 1')

note_sequence = [57, 59, 60, 62, 57, 59, 55, 57]

for note in note_sequence:
    time.sleep(0.25)
    outport.send(mido.Message('note_on', note=note, velocity = 100))
    time.sleep(0.25)
    outport.send(mido.Message('note_off', note=note, velocity = 100))
