import requests

url = 'https://api.devhub.virginia.edu/v1/courses'

data = requests.get(url).json()
print(data['class_schedules']['records'])
print(data['class_schedules']['columns'])
# ['subject', 'catalog_number', 'class_section', 'class_number', 'class_title', 'class_topic_formal_desc', 'instructor', 'enrollment_capacity', 'meeting_days', 'meeting_time_start', 'meeting_time_end', 'term', 'term_desc']
