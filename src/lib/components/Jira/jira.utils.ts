import { type DateValue, getLocalTimeZone } from '@internationalized/date';

export const formatDateForJira = (dateValue: DateValue | undefined): string | undefined => {
	if (!dateValue) return undefined;

	// Convert the DateValue to a JavaScript Date in the local time zone
	const jsDate = dateValue.toDate(getLocalTimeZone());

	// Format the date as ISO but strip the 'Z' (which represents UTC) and add the local timezone offset
	const isoString = jsDate.toISOString().replace('Z', '');

	// Get the timezone offset in hours and minutes (e.g., +05:30 or -04:00)
	const offset = -jsDate.getTimezoneOffset();
	const sign = offset >= 0 ? '+' : '-';
	const pad = (n: number) => String(Math.floor(Math.abs(n))).padStart(2, '0');
	const hours = pad(offset / 60);
	const minutes = pad(offset % 60);

	// Return the date in the format required (e.g., 2024-09-06T12:00:00.000+0000)
	return `${isoString}${sign}${hours}${minutes}`;
};
