export function formatBytes(bytes: number | bigint, decimals: number = 2): string {
	bytes = Number(bytes);
	if (!+bytes) return '0 Bytes';

	const k = 1024;
	const dm = decimals < 0 ? 0 : decimals;
	const sizes = ['Bytes', 'KiB', 'MiB', 'GiB', 'TiB', 'PiB', 'EiB', 'ZiB', 'YiB'];

	let i = Math.floor(Math.log(bytes) / Math.log(k));
	if (i < 0) {
		i = 0;
	}

	return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${sizes[i]}`;
}
