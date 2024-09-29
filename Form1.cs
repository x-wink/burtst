namespace Bursts
{
    public partial class MainForm : Form
    {
        private readonly MouseEventHandler mouseEventHandler;
        public MainForm()
        {
            InitializeComponent();
            mouseEventHandler = new MouseEventHandler();
        }
        public void Form1_Load(object sender, EventArgs e)
        {
            mouseEventHandler.Setup(10);
        }
        protected override void OnFormClosed(FormClosedEventArgs e)
        {
            mouseEventHandler.Destroy();
            base.OnFormClosed(e);
        }

        private void checkBox1_CheckedChanged(object sender, EventArgs e)
        {
            if (checkBox1.Checked)
            {
                mouseEventHandler.AddKey((int)MouseBtns.Left);
            }
            else
            {
                mouseEventHandler.DelKey((int)MouseBtns.Left);
            }
        }

        private void checkBox2_CheckedChanged(object sender, EventArgs e)
        {
            if(checkBox2.Checked)
            {
                mouseEventHandler.AddKey((int)MouseBtns.Middle);
            }
            else
            {
                mouseEventHandler.DelKey((int)MouseBtns.Middle);
            }
        }

        private void checkBox3_CheckedChanged(object sender, EventArgs e)
        {
            if (checkBox3.Checked)
            {
                mouseEventHandler.AddKey((int)MouseBtns.Right);
            }
            else
            {
                mouseEventHandler.DelKey((int)MouseBtns.Right);
            }
        }

    }
}
