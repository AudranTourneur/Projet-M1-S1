<script lang="ts">
    import { AppRail, AppRailAnchor, AppRailTile } from "@skeletonlabs/skeleton";
    import item from '../info.json';

    let overview = item.docker_info.overview;
    let containers=item.docker_info.containers;
    let volumes=item.docker_info.volumes;
    let images=item.docker_info.images;
    let currentTile: number = 0;
</script>

<div className="flex gap-4">
    {[
      [24, 32, 32, 16, 16],
      [32, 40, 56],
      [64, 32, 32],
    ].map((card, index) => (
      <div className="flex-1" key={index}>
        {card.map((height, index) => (
          <div
            className={`mb-4 h-${height} rounded-xl border-2 border-slate-400/10 bg-green-100 p-4 dark:bg-green-900`}
            key={index}
          ></div>
        ))}
      </div>
    ))}
  </div>
  