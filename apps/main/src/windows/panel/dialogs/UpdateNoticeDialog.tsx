import Button from '../components/Button';
import Markdown from '../components/Markdown';
import './dialog-base.css';
import './UpdateNoticeDialog.css';

export interface UpdateNoticeInfo {
  version: string;
  notes: string | null;
}

/**
 * 弹窗的两种形态，正文来源不同，不能混：
 * - `ready`：新版本已下载完成，正文是服务端给的新版本说明，可以立即重启安装。
 * - `current`：菜单里主动查看，正文是随包内联的**当前运行版本**那一节，只作阅读。
 */
export type UpdateNoticeMode = 'ready' | 'current';

interface Props {
  info: UpdateNoticeInfo;
  mode: UpdateNoticeMode;
  onClose: () => void;
  /** 仅 `ready` 形态：立即安装并重启。 */
  onRestart?: () => void;
  /** 安装命令进行中，按钮转圈并禁止重复点击。 */
  applying?: boolean;
}

export default function UpdateNoticeDialog({ info, mode, onClose, onRestart, applying }: Props) {
  const ready = mode === 'ready';
  return (
    <div className="update-notice-card">
      <div className="update-notice-header">
        <span className="update-badge">{ready ? '新版本' : '当前版本'}</span>
        <h2 className="update-notice-title">v{info.version}</h2>
        {ready ? (
          <p className="update-notice-hint">更新已下载完成，重启应用后生效</p>
        ) : (
          !info.notes && <p className="update-notice-hint">这个版本没有留下更新记录</p>
        )}
      </div>

      {/* current 形态下正文为空时改由 hint 兜底，不留一个空白的正文区 */}
      {info.notes && (
        <div className="update-notes-section">
          <p className="update-notes-label">更新内容</p>
          <div className="update-notes-body">
            <Markdown className="update-notes-text" source={info.notes} />
          </div>
        </div>
      )}

      <div className="update-notice-actions">
        {ready ? (
          <>
            <Button variant="outline" tone="neutral" onClick={onClose} disabled={applying}>
              稍后处理
            </Button>
            <Button loading={applying} onClick={onRestart}>
              {applying ? '正在安装…' : '重启并更新'}
            </Button>
          </>
        ) : (
          <Button className="update-notice-close" onClick={onClose}>
            知道了
          </Button>
        )}
      </div>
    </div>
  );
}
