using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Bursts
{
    internal interface EventHandler
    {
        void Setup(int step);
        void Destroy();
        void AddKey(int key);
        void DelKey(int key);
    }
}
