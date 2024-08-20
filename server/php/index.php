<?php
header('Content-Type: image/jpeg');
print(convert_lepton_to_jpeg(file_get_contents('images/' . $_GET['i']), 1));