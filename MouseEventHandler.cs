using System;
using System.Runtime.InteropServices;
using System.Windows.Forms;
using Timer = System.Windows.Forms.Timer;
namespace Bursts
{
    public enum MouseBtns
    {
        Left,
        Middle,
        Right
    }

    class MouseEventHandler : EventHandler
    {
        private readonly bool[] keys = { false, false, false };
        private readonly Timer timer;
        private readonly KeyboardHelper helper;

        public MouseEventHandler()
        {
            timer = new Timer();
            timer.Tick += Invoke;

            helper = new KeyboardHelper(null, (sender, e) => {
                switch (e.KeyCode)
                {
                    case Keys.Home:
                        MessageBox.Show("开始连发");
                        timer.Start();
                        break;
                    case Keys.End:
                        timer.Stop();
                        MessageBox.Show("停止连发");
                        break;
                }
                });
        }
        public void Setup(int step)
        {
            timer.Interval = step;
            helper.StartListening();
        }

        private void Invoke(object? sender, EventArgs e)
        {
            if (keys[(int)MouseBtns.Left])
            {
                KeyboardHelper.PressMouse(Keys.LButton);
            }
            if (keys[(int)MouseBtns.Middle])
            {
                KeyboardHelper.PressMouse(Keys.MButton);
            }
            if (keys[(int)MouseBtns.Right])
            {
                KeyboardHelper.PressMouse(Keys.RButton);
            }
        }

        public void Destroy()
        {
            timer.Stop();
            helper.StopListening();
        }

        public void AddKey(int key)
        {
            keys[key] = true;
        }

        public void DelKey(int key)
        {
            keys[key] = false;
        }
    }
}
