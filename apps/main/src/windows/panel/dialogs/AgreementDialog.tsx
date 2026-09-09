import { invoke } from '@tauri-apps/api/core';
import { useCallback, useEffect, useRef, useState } from 'react';
import eulaText from '../../../assets/EULA.md?raw';
import Button from '../components/Button';
import Markdown from '../components/Markdown';
import DialogShell from './DialogShell';
import './AgreementDialog.css';

interface Props {
  onAgreed: () => void;
}

/** 判定「已滚到底」的容差（px），避免子像素误差导致永远差一点点。 */
const BOTTOM_SLACK = 16;

export default function AgreementPage({ onAgreed }: Props) {
  const [scrolledToBottom, setScrolledToBottom] = useState(false);
  const [agreeing, setAgreeing] = useState(false);
  const [agreeError, setAgreeError] = useState('');
  const bodyRef = useRef<HTMLDivElement>(null);

  // 滚动发生在 DialogShell 的 .fb-dialog__body 上，必须监听那一层。此前挂在协议正文外面
  // 包的一层 div 上，那个 div 高度随内容撑开、自身从不滚动，scrollHeight 恒等于
  // clientHeight，于是挂载时就判定「已读完」，同意按钮一直可点，门禁形同虚设。
  useEffect(() => {
    const el = bodyRef.current;
    if (!el) return;
    const checkBottom = () => {
      // 尚未完成布局时两个高度都是 0，`0 <= 0 + 容差` 会误判成「内容不足一屏」直接放行
      if (el.clientHeight === 0) return;
      // 内容不足一屏时无从滚动，直接视为读完
      if (el.scrollHeight <= el.clientHeight + BOTTOM_SLACK) {
        setScrolledToBottom(true);
        return;
      }
      if (el.scrollHeight - el.scrollTop - el.clientHeight < BOTTOM_SLACK) {
        setScrolledToBottom(true);
      }
    };
    checkBottom();
    el.addEventListener('scroll', checkBottom, { passive: true });
    // 窗口缩放会改变可滚动高度，只在挂载时量一次会误判；正文本身也一并观察，
    // 以防将来协议改成异步加载。
    const observer = new ResizeObserver(checkBottom);
    observer.observe(el);
    if (el.firstElementChild) observer.observe(el.firstElementChild);
    return () => {
      el.removeEventListener('scroll', checkBottom);
      observer.disconnect();
    };
  }, []);

  const handleAgree = useCallback(async () => {
    setAgreeing(true);
    try {
      await invoke('agree_license');
      onAgreed();
    } catch {
      setAgreeing(false);
      setAgreeError('操作失败，请重试');
    }
  }, [onAgreed]);

  const handleReject = useCallback(async () => {
    await invoke('exit_app');
  }, []);

  return (
    <DialogShell
      className="agreement-card"
      title="用户协议"
      labelId="agreement-title"
      bodyRef={bodyRef}
      footer={
        <>
          <p className="agreement-hint">
            {agreeError || (scrolledToBottom ? '已阅读完毕' : '请滚动阅读协议全文')}
          </p>
          <Button
            variant="outline"
            tone="neutral"
            className="agreement-action"
            onClick={handleReject}
          >
            不同意并退出
          </Button>
          <Button
            className="agreement-action"
            loading={agreeing}
            disabled={!scrolledToBottom}
            onClick={handleAgree}
          >
            {agreeing ? '处理中…' : '同意并继续'}
          </Button>
        </>
      }
    >
      <Markdown className="agreement-text" source={eulaText} />
    </DialogShell>
  );
}
