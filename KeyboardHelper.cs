using System;
using System.Diagnostics;
using System.Runtime.InteropServices;
using System.Windows.Forms;

namespace Bursts
{
    public class KeyboardHelper
    {
        // 导入 user32.dll 中的 keybd_event 函数
        [DllImport("user32.dll", CharSet = CharSet.Auto, CallingConvention = CallingConvention.StdCall)]
        public static extern void keybd_event(byte bVk, byte bScan, uint dwFlags, UIntPtr dwExtraInfo);

        // 导入 user32.dll 中的 mouse_event 函数
        [DllImport("user32.dll", CharSet = CharSet.Auto, CallingConvention = CallingConvention.StdCall)]
        public static extern void mouse_event(uint dwFlags, uint dx, uint dy, uint cButtons, UIntPtr dwExtraInfo);

        // 导入 user32.dll 中的 SetWindowsHookEx 函数
        [DllImport("user32.dll", CharSet = CharSet.Auto, SetLastError = true)]
        private static extern IntPtr SetWindowsHookEx(int idHook, LowLevelKeyboardProc lpfn, IntPtr hMod, uint dwThreadId);

        // 导入 user32.dll 中的 UnhookWindowsHookEx 函数
        [DllImport("user32.dll", CharSet = CharSet.Auto, SetLastError = true)]
        [return: MarshalAs(UnmanagedType.Bool)]
        private static extern bool UnhookWindowsHookEx(IntPtr hhk);

        // 导入 user32.dll 中的 CallNextHookEx 函数
        [DllImport("user32.dll", CharSet = CharSet.Auto, SetLastError = true)]
        private static extern IntPtr CallNextHookEx(IntPtr hhk, int nCode, IntPtr wParam, IntPtr lParam);

        // 导入 kernel32.dll 中的 GetModuleHandle 函数
        [DllImport("kernel32.dll", CharSet = CharSet.Auto, SetLastError = true)]
        private static extern IntPtr GetModuleHandle(string lpModuleName);

        // 定义键盘钩子常量
        private const int WH_KEYBOARD_LL = 13;
        private const int WM_KEYDOWN = 0x0100;
        private const int WM_KEYUP = 0x0101;

        // 定义键盘事件常量
        private const uint MOUSEEVENTF_LEFTDOWN = 0x02;
        private const uint MOUSEEVENTF_LEFTUP = 0x04;
        private const uint MOUSEEVENT_RIGHTDOWN = 0x08;
        private const uint MOUSEEVENTF_RIGHTUP = 0x10;
        private const uint MOUSEEVENT_MIDDLEDOWN = 0x20;
        private const uint MOUSEEVENTF_MIDDLEUP = 0x40;
        private const uint KEYEVENTF_KEYDOWN = 0x0000;
        private const uint KEYEVENTF_KEYUP = 0x0002;

        // 定义键盘钩子回调函数
        private delegate IntPtr LowLevelKeyboardProc(int nCode, IntPtr wParam, IntPtr lParam);

        private static IntPtr _hookID = IntPtr.Zero;

        private event EventHandler<KeyEventArgs> KeyDown;
        private event EventHandler<KeyEventArgs> KeyUp;

        public KeyboardHelper(EventHandler<KeyEventArgs>? down, EventHandler<KeyEventArgs>? up)
        {
            KeyDown = down;
            KeyUp = up;
        }

        public void StartListening()
        {
            _hookID = SetHook(HookCallback);
        }

        public void StopListening()
        {
            UnhookWindowsHookEx(_hookID);
        }

        private static IntPtr SetHook(LowLevelKeyboardProc proc)
        {
            using Process curProcess = Process.GetCurrentProcess();
            using ProcessModule curModule = curProcess.MainModule;
            return SetWindowsHookEx(WH_KEYBOARD_LL, proc, GetModuleHandle(curModule.ModuleName), 0);
        }

        private IntPtr HookCallback(int nCode, IntPtr wParam, IntPtr lParam)
        {
            if (nCode >= 0)
            {
                int vkCode = Marshal.ReadInt32(lParam);
                if (wParam == (IntPtr)WM_KEYDOWN)
                {
                    KeyDown?.Invoke(null, new KeyEventArgs((Keys)vkCode));
                }
                else if (wParam == (IntPtr)WM_KEYUP)
                {
                    KeyUp?.Invoke(null, new KeyEventArgs((Keys)vkCode));
                }
            }
            return CallNextHookEx(_hookID, nCode, wParam, lParam);
        }

        public static void PressKeyboard(Keys key)
        {
            keybd_event((byte)key, 0, KEYEVENTF_KEYDOWN, nuint.Zero);
            keybd_event((byte)key, 0, KEYEVENTF_KEYUP, nuint.Zero);
        }
        public static void PressMouse(Keys key)
        {
            if(key == Keys.LButton)
            {
                mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, UIntPtr.Zero);
                mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, UIntPtr.Zero);
            }
            else if (key == Keys.RButton)
            {
                mouse_event(MOUSEEVENT_RIGHTDOWN, 0, 0, 0, UIntPtr.Zero);
                mouse_event(MOUSEEVENTF_RIGHTUP, 0, 0, 0, UIntPtr.Zero);
            }
            else if (key == Keys.MButton)
            {
                mouse_event(MOUSEEVENT_MIDDLEDOWN, 0, 0, 0, UIntPtr.Zero);
                mouse_event(MOUSEEVENTF_MIDDLEUP, 0, 0, 0, UIntPtr.Zero);
            }
        }
    }
}
