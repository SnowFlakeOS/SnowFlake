function startTime () {
	var today = new Date();
	var AMPM = null;
	var year = today.getFullYear();
	var month = today.getMonth() + 1;
	var date = today.getDate();
	var hour = today.getHours();
	var minute = today.getMinutes();
	var second = today.getSeconds();

	if (hour > 12) {
		AMPM = "오후";
		hour = hour - 12;
	}

	else if (hour == 0) {
		AMPM = "오전";
		hour = hour + 12;
	}

	else if (hour == 12) {
		AMPM = "오후";
	}

	else {
		AMPM = "오전";
	}

	minute = checkTime(minute);
	second = checkTime(second);
	month = checkTime(month);

	document.getElementById('clock').innerHTML = AMPM + " " + hour + ":" + minute;
	document.getElementById('date').innerHTML = year + "-" + month + "-" + date;
	var t = setTimeout(startTime, 500);
}

function checkTime (i) {
	if (i < 10) {
		// 숫자가 0보다 작으면 앞에 0을 붙여준다
		i = "0" + i;
	}
	return i;
}
