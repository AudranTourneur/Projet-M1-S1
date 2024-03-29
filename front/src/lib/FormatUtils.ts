export function formatBytes(bytes: number | bigint, decimals: number = 2): string {
    bytes = Number(bytes);
    if (!+bytes) return '0 Bytes';

    const k = 1024;
    const dm = decimals < 0 ? 0 : decimals;
    const sizes = ['Bytes', 'KiB', 'MiB', 'GiB', 'TiB', 'PiB', 'EiB', 'ZiB', 'YiB'];

    const i = Math.floor(Math.log(bytes) / Math.log(k));

    return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${sizes[i]}`;
}

export function formatCreatedDate(sec: number): string {
    const date = new Date(sec * 1000);
    const iso = date.toISOString();
    return iso.split('.')[0].replace('T', ' ');
}