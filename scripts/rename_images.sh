cd ../public/clara/img
for i in *.jpeg_large; do
  mv "$i" "${i%.jpeg_large}.jpg"
done