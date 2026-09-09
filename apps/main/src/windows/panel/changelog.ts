import changelogText from '../../../../../CHANGELOG.md?raw';

/**
 * 从随包内联的 `CHANGELOG.md` 中取出指定版本那一节的正文。
 *
 * 菜单里的「更新公告」看的是**当前运行版本**做了什么，来源必须是本地这份，而不是
 * 检查更新拿到的新版本 `body`——那份属于还没装上的版本，只在下载完成的弹窗里出现。
 *
 * 分节规则与 `scripts/extract-changelog.ts` 保持一致：`## [版本号]` 起，到下一个
 * `## [` 止；CI 发版时用同一规则提取 Release 正文，两边看到的内容因此一致。
 */
export function changelogSection(version: string): string | null {
  if (!version) return null;
  const lines = changelogText.replace(/\r\n?/g, '\n').split('\n');
  const start = lines.findIndex((line) => line.startsWith(`## [${version}]`));
  if (start === -1) return null;

  let end = lines.findIndex((line, i) => i > start && line.startsWith('## ['));
  if (end === -1) end = lines.length;

  // 丢掉版本号那一行：弹窗标题已经写了版本，正文再写一遍是重复
  const body = lines
    .slice(start + 1, end)
    .join('\n')
    .trim();
  return body || null;
}
