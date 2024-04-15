export function getContainerActionsFromStatus(status: string) : {statusIcon: 'running'|'paused'|'stopped', canBeStarted: boolean, canBeStopped: boolean} {
	if (status.includes('Paused')) return {statusIcon: 'paused', canBeStarted: false, canBeStopped: true};
	else if (status.includes('Up')) return {statusIcon: 'running', canBeStarted: false, canBeStopped: true};
	else return {statusIcon: 'stopped', canBeStarted: true, canBeStopped: false};
}