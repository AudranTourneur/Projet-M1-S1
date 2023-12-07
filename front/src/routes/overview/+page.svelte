<script lang="ts">
    import { AppRail, AppRailAnchor, AppRailTile } from "@skeletonlabs/skeleton";


    let currentTile: number = 0;
</script>

<div class="overflow-x-hidden">
    <div>
        <h1>Overview</h1>

    </div>

    <div class="h-screen">
        <AppRail>
            <AppRailTile bind:group={currentTile} name="tile-1" value={0} title="tile-1">
                <span>Images</span>
            </AppRailTile>
            <AppRailTile bind:group={currentTile} name="tile-2" value={1} title="tile-2">
                <span>Containers</span>
            </AppRailTile>
            <AppRailTile bind:group={currentTile} name="tile-3" value={2} title="tile-3">
                <span>Volumes</span>
            </AppRailTile>
        </AppRail>
    </div>
</div>