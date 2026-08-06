### Требуемое окружение
Папка models содержащая:
- eslav_pp-ocrv5_mobile_rec.onnx
- pp-ocrv5_mobile_det.onnx
- ppocrv5_eslav_dict.txt

Папка logs содержащая:
- Папку images
- Папку program

В корневой папке:
- settings, содержащий "PTT\tpath_to_token\nADM_ID\tadmin_discord_id\nPTD\tpath_to_dictionary"
- Опционально token, содрежащий только discord токен и words.txt, что является словарём с весами.

Запускать как .exe, молиться чтобы работало. Логи в папке logs

### На текущий момент
Будет просто пинговать роль или человека указанного в settings, если обнаружит что-то подозрительное.

#### TODO
- Сделать настройку веса на срабатывание
- Прочие действия
- Настройка с команд в диске
