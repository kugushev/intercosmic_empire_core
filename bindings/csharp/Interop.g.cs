// Automatically generated by Interoptopus.

#pragma warning disable 0105
using System;
using System.Collections;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using AK.Scripts.Core.Native;
using UnityEngine;
#pragma warning restore 0105

namespace AK.Scripts.Core.Native
{
    public static partial class Interop
    {
        public const string NativeLib = "intercosmic_empire";

        static Interop()
        {
        }


        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_hello_from_rust")]
        public static extern int ice_hello_from_rust(int a);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_steering_seek")]
        public static extern FFIResult ice_steering_seek(ref Vector3 position, ref Vector3 target, float mass, float max_speed, ref Vector3 current_velocity, out Vector3 output);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_init_game")]
        public static extern FFIResult ice_init_game(ref IntPtr context);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_close_game")]
        public static extern FFIResult ice_close_game(ref IntPtr context);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_register_stellar_system")]
        public static extern void ice_register_stellar_system(IntPtr context, StellarSystemId id, Sun sun, StellarSystemParameters parameters);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_register_planet")]
        public static extern void ice_register_planet(IntPtr context, StellarSystemId id, Planet planet);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_start_battle")]
        public static extern void ice_start_battle(IntPtr context, BattleParameters parameters);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_battle_open_warp_gate")]
        public static extern void ice_battle_open_warp_gate(IntPtr context, WarpGate warp_gate);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_finish_battle")]
        public static extern void ice_finish_battle(IntPtr context);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_battle_update")]
        public static extern IntPtr ice_battle_update(IntPtr context, float delta_time);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_get_battle_view_model")]
        public static extern IntPtr ice_get_battle_view_model(IntPtr context);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_get_battle_stellar_system_view_model")]
        public static extern StellarSystemViewModel ice_get_battle_stellar_system_view_model(IntPtr context);

    }

    public enum FFIResult
    {
        Ok = 0,
        NullPointerError = 1,
        NotNullPointerError = 2,
    }

    public enum Faction
    {
        White = 0,
        Red = 1,
        Green = 2,
        Blue = 3,
        Grey = 4,
    }

    public enum PlanetSize
    {
        Mercury = 0,
        Mars = 1,
        Earth = 2,
        Uranus = 3,
        Saturn = 4,
        Jupiter = 5,
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct BattleParameters
    {
        public int seed;
        public StellarSystemId stellar_system_id;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct BattleViewModel
    {
        public Vector3 test_position;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct Orbit
    {
        public float radius;
        public float alpha_rotation;
        public float beta_rotation;
        public int start_day;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct Planet
    {
        public PlanetInfo info;
        public Vector3 position;
        public Faction faction;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct PlanetId
    {
        public int x0;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct PlanetInfo
    {
        public PlanetId id;
        public Orbit orbit;
        public PlanetSize size;
        public Production production;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct Production
    {
        public float amount_per_second;
        public float max_product;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct StellarSystemId
    {
        public int x0;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct StellarSystemParameters
    {
        public float system_radius;
        public float min_distance_to_sun;
        public Vector3 center;
        public float sun_min_radius;
        public float sun_max_radius;
        public int min_planets;
        public int max_planets;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct StellarSystemViewModel
    {
        public StellarSystemId id;
        public IntPtr sun;
        public IntPtr parameters;
        public SlicePlanet planets;
        public SliceWarpGate warp_gates;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct Sun
    {
        public Vector3 position;
        public float radius;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct WarpGate
    {
        public Vector3 position;
        public Faction faction;
        public Production production;
    }

    ///A pointer to an array of data someone else owns which may not be modified.
    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct SlicePlanet
    {
        ///Pointer to start of immutable data.
        IntPtr data;
        ///Number of elements.
        ulong len;
    }

    public partial struct SlicePlanet : IEnumerable<Planet>
    {
        public SlicePlanet(GCHandle handle, ulong count)
        {
            this.data = handle.AddrOfPinnedObject();
            this.len = count;
        }
        public SlicePlanet(IntPtr handle, ulong count)
        {
            this.data = handle;
            this.len = count;
        }
        public Planet this[int i]
        {
            get
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                var size = Marshal.SizeOf(typeof(Planet));
                var ptr = new IntPtr(data.ToInt64() + i * size);
                return Marshal.PtrToStructure<Planet>(ptr);
            }
        }
        public Planet[] Copied
        {
            get
            {
                var rval = new Planet[len];
                for (var i = 0; i < (int) len; i++) {
                    rval[i] = this[i];
                }
                return rval;
            }
        }
        public int Count => (int) len;
        public IEnumerator<Planet> GetEnumerator()
        {
            for (var i = 0; i < (int)len; ++i)
            {
                yield return this[i];
            }
        }
        IEnumerator IEnumerable.GetEnumerator()
        {
            return this.GetEnumerator();
        }
    }


    ///A pointer to an array of data someone else owns which may not be modified.
    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct SliceWarpGate
    {
        ///Pointer to start of immutable data.
        IntPtr data;
        ///Number of elements.
        ulong len;
    }

    public partial struct SliceWarpGate : IEnumerable<WarpGate>
    {
        public SliceWarpGate(GCHandle handle, ulong count)
        {
            this.data = handle.AddrOfPinnedObject();
            this.len = count;
        }
        public SliceWarpGate(IntPtr handle, ulong count)
        {
            this.data = handle;
            this.len = count;
        }
        public WarpGate this[int i]
        {
            get
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                var size = Marshal.SizeOf(typeof(WarpGate));
                var ptr = new IntPtr(data.ToInt64() + i * size);
                return Marshal.PtrToStructure<WarpGate>(ptr);
            }
        }
        public WarpGate[] Copied
        {
            get
            {
                var rval = new WarpGate[len];
                for (var i = 0; i < (int) len; i++) {
                    rval[i] = this[i];
                }
                return rval;
            }
        }
        public int Count => (int) len;
        public IEnumerator<WarpGate> GetEnumerator()
        {
            for (var i = 0; i < (int)len; ++i)
            {
                yield return this[i];
            }
        }
        IEnumerator IEnumerable.GetEnumerator()
        {
            return this.GetEnumerator();
        }
    }




    public class InteropException<T> : Exception
    {
        public T Error { get; private set; }

        public InteropException(T error): base($"Something went wrong: {error}")
        {
            Error = error;
        }
    }

}
