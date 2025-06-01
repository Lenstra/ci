select first_name, last_name, email_address
from users
where active = true
order by last_name asc
;
