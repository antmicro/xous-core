//
// Copyright (c) 2010-2020 Antmicro
//
// This file is licensed under the MIT License.
// Full license text is available in 'licenses/MIT.txt'.
//

using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using Antmicro.Renode.Core;
using Antmicro.Renode.Core.Structure.Registers;
using Antmicro.Renode.Logging;
using Antmicro.Renode.Peripherals.I2C;
using Antmicro.Renode.Peripherals.Sensor;
using Antmicro.Renode.Utilities;
using Antmicro.Renode.Exceptions;

namespace Antmicro.Renode.Peripherals.Sensors
{
    public class BQ27421 : II2CPeripheral, IProvidesRegisterCollection<ByteRegisterCollection>, ISensor, ITemperatureSensor
    {
        public BQ27421()
        {
            RegistersCollection = new ByteRegisterCollection(this);
            IRQ = new GPIO();
            DefineRegisters();
        }

        public void FinishTransmission(){}

        public void Reset()
        {
            RegistersCollection.Reset();
            IRQ.Set(false);
            regAddress = 0;
        }

        public void Write(byte[] data)
        {
            if(data.Length == 0 || data.Length == 1)
            {
                this.Log(LogLevel.Warning, "Unexpected write with no data");
                return;
            }

            // The first byte contains our address.
            this.Log(LogLevel.Noisy, "Write with {0} bytes of data: {1}", data.Length, Misc.PrettyPrintCollectionHex(data));

            // // The second byte contains the register address
            // regAddress = (Registers)data[1];

            // if(data.Length == 2)
            // {
            //     this.Log(LogLevel.Noisy, "Preparing to read register {0} (0x{0:X})", regAddress);
            //     readyPending.Value = true;
            //     UpdateInterrupts();
            //     return;
            // }
        }

        public byte[] Read(int count)
        {
            this.Log(LogLevel.Error, "Reading {0} bytes from register {1} (0x{1:X})", count, regAddress);
            var result = new byte[count];
            // readyPending.Value = false;
            // UpdateInterrupts();
            if (count >= 1) {
                result[0] = (byte)(response & 0xff);
            }
            if (count >= 2) {
                result[1] = (byte)((response >> 8) & 0xff);
            }
            // for(var i = 0; i < result.Length; i++)
            // {
            //     result[i] = RegistersCollection.Read((byte)regAddress);
            //     this.Log(LogLevel.Error, "Read value {0}", result[i]);
            //     RegistersAutoIncrement();
            // }
            return result;
        }
        public decimal Temperature { get; set; }

        public GPIO IRQ { get; }
        public ByteRegisterCollection RegistersCollection { get; }

        private void DefineRegisters()
        {
            Registers.Temp.Define(this)
                .WithValueField(0, 8, FieldMode.Read, name: "TEMPERATURE_SENSOR", valueProviderCallback: _ => TwoComplementSignConvert(Temperature));
        }

        private void RegistersAutoIncrement()
        {
        }

        // private void UpdateInterrupts()
        // {
        //     var status = readyEnabled.Value && readyPending.Value;
        //     this.Log(LogLevel.Noisy, "Setting IRQ to {0}", status);
        //     IRQ.Set(status);
        // }

        private byte TwoComplementSignConvert(decimal temp)
        {
            byte tempAsByte = Decimal.ToByte(temp);
            if(temp < 0)
            {
                byte twoComplementTemp = (byte)(~tempAsByte + 1);
                return twoComplementTemp;
            }
            return tempAsByte;
        }

        private IFlagRegisterField readyPending;
        private IFlagRegisterField readyEnabled;
        private IFlagRegisterField highFreqDataRateMode;
        private IValueRegisterField outDataRate;
        private IValueRegisterField fullScale;
        private Registers regAddress;

        private ushort controlStatus;
        private ushort response;

        private enum Registers : byte
        {
            Cntl = 0x00,
            // Reserved: 0x01
            Temp = 0x02,
            // Reserved: 0x03
            Volt = 0x04,
            // Reserved: 0x05
            Flag = 0x06,
            // Reserved: 0x07
            NomCap = 0x08,
            // Reserved: 0x09
            FullCap = 0x0A,
            // Reserved: 0x0B
            Rm = 0x0C,
            // Reserved: 0x0d
            Fcc = 0x0e,
            // Reserved: 0x0f
            AvgCur = 0x10,
            // Reserved: 0x11
            SbyCur = 0x12,
            // Reserved: 0x13
            MaxCur = 0x14,
            // Reserved: 0x15 - 0x17
            AvgPwr = 0x18,
            // Reserved: 0x19 = 0x1b
            Soc = 0x1c,
            // Reserved: 0x1d
            IntTemp = 0x1e,
            // Reserved: 0x1f
            Soh = 0x20,
        }
    }
}
